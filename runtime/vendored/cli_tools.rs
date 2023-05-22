// https://github.com/denoland/deno/blob/v1.33.3/cli/tools/test.rs
//
// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use crate::fmt_errors::format_js_error;
use deno_core::error::JsError;

fn abbreviate_test_error(js_error: &JsError) -> JsError {
    let mut js_error = js_error.clone();
    let frames = std::mem::take(&mut js_error.frames);

    // check if there are any stack frames coming from user code
    let should_filter = frames.iter().any(|f| {
        if let Some(file_name) = &f.file_name {
            !(file_name.starts_with("[ext:") || file_name.starts_with("ext:"))
        } else {
            true
        }
    });

    if should_filter {
        let mut frames = frames
            .into_iter()
            .rev()
            .skip_while(|f| {
                if let Some(file_name) = &f.file_name {
                    file_name.starts_with("[ext:") || file_name.starts_with("ext:")
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        frames.reverse();
        js_error.frames = frames;
    } else {
        js_error.frames = frames;
    }

    js_error.cause = js_error
        .cause
        .as_ref()
        .map(|e| Box::new(abbreviate_test_error(e)));
    js_error.aggregated = js_error
        .aggregated
        .as_ref()
        .map(|es| es.iter().map(abbreviate_test_error).collect());
    js_error
}

// This function prettifies `JsError` and applies some changes specifically for
// test runner purposes:
//
// - filter out stack frames:
//   - if stack trace consists of mixed user and internal code, the frames
//     below the first user code frame are filtered out
//   - if stack trace consists only of internal code it is preserved as is
pub fn format_test_error(js_error: &JsError) -> String {
    let mut js_error = abbreviate_test_error(js_error);
    js_error.exception_message = js_error
        .exception_message
        .trim_start_matches("Uncaught ")
        .to_string();
    format_js_error(&js_error)
}
