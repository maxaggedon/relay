/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use extract_graphql::{extract, JavaScriptSourceFeature};
use fixture_tests::Fixture;

pub fn transform_fixture(fixture: &Fixture<'_>) -> Result<String, String> {
    let features = extract(fixture.content);
    Ok(features
        .into_iter()
        .map(|feature| match feature {
            JavaScriptSourceFeature::Docblock(s) => format!(
                "docblock - line: {}, column: {}, text: <{}>",
                s.line_index, s.column_index, s.text
            ),
            JavaScriptSourceFeature::GraphQL(s) => format!(
                "graphql - line: {}, column: {}, text: <{}>",
                s.line_index, s.column_index, s.text
            ),
        })
        .collect::<Vec<_>>()
        .join("\n"))
}
