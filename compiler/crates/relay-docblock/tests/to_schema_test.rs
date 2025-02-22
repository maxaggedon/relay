/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<b0c707529f8b045554b20820c2e3e271>>
 */

mod to_schema;

use to_schema::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn client_edge_relay_resolver() {
    let input = include_str!("to_schema/fixtures/client-edge-relay-resolver.js");
    let expected = include_str!("to_schema/fixtures/client-edge-relay-resolver.expected");
    test_fixture(transform_fixture, "client-edge-relay-resolver.js", "to_schema/fixtures/client-edge-relay-resolver.expected", input, expected);
}

#[test]
fn client_edge_to_non_null_plural_server_object_relay_resolver_invalid() {
    let input = include_str!("to_schema/fixtures/client-edge-to-non-null-plural-server-object-relay-resolver.invalid.js");
    let expected = include_str!("to_schema/fixtures/client-edge-to-non-null-plural-server-object-relay-resolver.invalid.expected");
    test_fixture(transform_fixture, "client-edge-to-non-null-plural-server-object-relay-resolver.invalid.js", "to_schema/fixtures/client-edge-to-non-null-plural-server-object-relay-resolver.invalid.expected", input, expected);
}

#[test]
fn client_edge_to_plural_server_object_relay_resolver_invalid() {
    let input = include_str!("to_schema/fixtures/client-edge-to-plural-server-object-relay-resolver.invalid.js");
    let expected = include_str!("to_schema/fixtures/client-edge-to-plural-server-object-relay-resolver.invalid.expected");
    test_fixture(transform_fixture, "client-edge-to-plural-server-object-relay-resolver.invalid.js", "to_schema/fixtures/client-edge-to-plural-server-object-relay-resolver.invalid.expected", input, expected);
}

#[test]
fn legacy_relay_resolver_with_root_fragment_on_model() {
    let input = include_str!("to_schema/fixtures/legacy-relay-resolver-with-root-fragment-on-model.js");
    let expected = include_str!("to_schema/fixtures/legacy-relay-resolver-with-root-fragment-on-model.expected");
    test_fixture(transform_fixture, "legacy-relay-resolver-with-root-fragment-on-model.js", "to_schema/fixtures/legacy-relay-resolver-with-root-fragment-on-model.expected", input, expected);
}

#[test]
fn relay_resolver() {
    let input = include_str!("to_schema/fixtures/relay-resolver.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver.expected");
    test_fixture(transform_fixture, "relay-resolver.js", "to_schema/fixtures/relay-resolver.expected", input, expected);
}

#[test]
fn relay_resolver_deprecated() {
    let input = include_str!("to_schema/fixtures/relay-resolver-deprecated.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-deprecated.expected");
    test_fixture(transform_fixture, "relay-resolver-deprecated.js", "to_schema/fixtures/relay-resolver-deprecated.expected", input, expected);
}

#[test]
fn relay_resolver_deprecated_no_description() {
    let input = include_str!("to_schema/fixtures/relay-resolver-deprecated-no-description.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-deprecated-no-description.expected");
    test_fixture(transform_fixture, "relay-resolver-deprecated-no-description.js", "to_schema/fixtures/relay-resolver-deprecated-no-description.expected", input, expected);
}

#[test]
fn relay_resolver_id_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-id.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-id.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-id.invalid.js", "to_schema/fixtures/relay-resolver-id.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_implementing_a_field_defined_by_grandparent_interface() {
    let input = include_str!("to_schema/fixtures/relay-resolver-implementing-a-field-defined-by-grandparent-interface.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-implementing-a-field-defined-by-grandparent-interface.expected");
    test_fixture(transform_fixture, "relay-resolver-implementing-a-field-defined-by-grandparent-interface.js", "to_schema/fixtures/relay-resolver-implementing-a-field-defined-by-grandparent-interface.expected", input, expected);
}

#[test]
fn relay_resolver_implementing_a_field_defined_by_parent_interface() {
    let input = include_str!("to_schema/fixtures/relay-resolver-implementing-a-field-defined-by-parent-interface.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-implementing-a-field-defined-by-parent-interface.expected");
    test_fixture(transform_fixture, "relay-resolver-implementing-a-field-defined-by-parent-interface.js", "to_schema/fixtures/relay-resolver-implementing-a-field-defined-by-parent-interface.expected", input, expected);
}

#[test]
fn relay_resolver_named_export() {
    let input = include_str!("to_schema/fixtures/relay-resolver-named-export.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-named-export.expected");
    test_fixture(transform_fixture, "relay-resolver-named-export.js", "to_schema/fixtures/relay-resolver-named-export.expected", input, expected);
}

#[test]
fn relay_resolver_on_interface() {
    let input = include_str!("to_schema/fixtures/relay-resolver-on-interface.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-on-interface.expected");
    test_fixture(transform_fixture, "relay-resolver-on-interface.js", "to_schema/fixtures/relay-resolver-on-interface.expected", input, expected);
}

#[test]
fn relay_resolver_on_interface_implementing_a_field_defined_by_parent_interface() {
    let input = include_str!("to_schema/fixtures/relay-resolver-on-interface-implementing-a-field-defined-by-parent-interface.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-on-interface-implementing-a-field-defined-by-parent-interface.expected");
    test_fixture(transform_fixture, "relay-resolver-on-interface-implementing-a-field-defined-by-parent-interface.js", "to_schema/fixtures/relay-resolver-on-interface-implementing-a-field-defined-by-parent-interface.expected", input, expected);
}

#[test]
fn relay_resolver_on_interface_with_type_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-on-interface-with-type.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-on-interface-with-type.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-on-interface-with-type.invalid.js", "to_schema/fixtures/relay-resolver-on-interface-with-type.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_on_invalid_interface_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-on-invalid-interface.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-on-invalid-interface.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-on-invalid-interface.invalid.js", "to_schema/fixtures/relay-resolver-on-invalid-interface.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_on_invalid_type_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-on-invalid-type.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-on-invalid-type.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-on-invalid-type.invalid.js", "to_schema/fixtures/relay-resolver-on-invalid-type.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_on_type_with_interface_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-on-type-with-interface.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-on-type-with-interface.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-on-type-with-interface.invalid.js", "to_schema/fixtures/relay-resolver-on-type-with-interface.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object.js", "to_schema/fixtures/relay-resolver-strong-object.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_implements() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-implements.js", "to_schema/fixtures/relay-resolver-strong-object-with-implements.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_implements_interface_bad_id_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-bad-id.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-bad-id.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-implements-interface-bad-id.invalid.js", "to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-bad-id.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_implements_interface_no_id_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-no-id.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-no-id.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-implements-interface-no-id.invalid.js", "to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-no-id.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_implements_interface_non_interface() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-non-interface.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-non-interface.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-implements-interface-non-interface.js", "to_schema/fixtures/relay-resolver-strong-object-with-implements-interface-non-interface.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_implements_non_existing_type() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-non-existing-type.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-non-existing-type.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-implements-non-existing-type.js", "to_schema/fixtures/relay-resolver-strong-object-with-implements-non-existing-type.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_implements_server_interface_invalid() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-server-interface.invalid.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-implements-server-interface.invalid.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-implements-server-interface.invalid.js", "to_schema/fixtures/relay-resolver-strong-object-with-implements-server-interface.invalid.expected", input, expected);
}

#[test]
fn relay_resolver_strong_object_with_multiple_implements() {
    let input = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-multiple-implements.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-strong-object-with-multiple-implements.expected");
    test_fixture(transform_fixture, "relay-resolver-strong-object-with-multiple-implements.js", "to_schema/fixtures/relay-resolver-strong-object-with-multiple-implements.expected", input, expected);
}

#[test]
fn relay_resolver_with_args() {
    let input = include_str!("to_schema/fixtures/relay-resolver-with-args.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-with-args.expected");
    test_fixture(transform_fixture, "relay-resolver-with-args.js", "to_schema/fixtures/relay-resolver-with-args.expected", input, expected);
}

#[test]
fn relay_resolver_with_field_and_fragment_args() {
    let input = include_str!("to_schema/fixtures/relay-resolver-with-field-and-fragment-args.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-with-field-and-fragment-args.expected");
    test_fixture(transform_fixture, "relay-resolver-with-field-and-fragment-args.js", "to_schema/fixtures/relay-resolver-with-field-and-fragment-args.expected", input, expected);
}

#[test]
fn relay_resolver_with_field_args() {
    let input = include_str!("to_schema/fixtures/relay-resolver-with-field-args.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-with-field-args.expected");
    test_fixture(transform_fixture, "relay-resolver-with-field-args.js", "to_schema/fixtures/relay-resolver-with-field-args.expected", input, expected);
}

#[test]
fn relay_resolver_with_output_type() {
    let input = include_str!("to_schema/fixtures/relay-resolver-with-output-type.js");
    let expected = include_str!("to_schema/fixtures/relay-resolver-with-output-type.expected");
    test_fixture(transform_fixture, "relay-resolver-with-output-type.js", "to_schema/fixtures/relay-resolver-with-output-type.expected", input, expected);
}

#[test]
fn terse_relay_resolver() {
    let input = include_str!("to_schema/fixtures/terse-relay-resolver.js");
    let expected = include_str!("to_schema/fixtures/terse-relay-resolver.expected");
    test_fixture(transform_fixture, "terse-relay-resolver.js", "to_schema/fixtures/terse-relay-resolver.expected", input, expected);
}

#[test]
fn terse_relay_resolver_id_invalid() {
    let input = include_str!("to_schema/fixtures/terse-relay-resolver-id.invalid.js");
    let expected = include_str!("to_schema/fixtures/terse-relay-resolver-id.invalid.expected");
    test_fixture(transform_fixture, "terse-relay-resolver-id.invalid.js", "to_schema/fixtures/terse-relay-resolver-id.invalid.expected", input, expected);
}

#[test]
fn terse_relay_resolver_interface() {
    let input = include_str!("to_schema/fixtures/terse-relay-resolver-interface.js");
    let expected = include_str!("to_schema/fixtures/terse-relay-resolver-interface.expected");
    test_fixture(transform_fixture, "terse-relay-resolver-interface.js", "to_schema/fixtures/terse-relay-resolver-interface.expected", input, expected);
}

#[test]
fn terse_relay_resolver_non_existent_type_invalid() {
    let input = include_str!("to_schema/fixtures/terse-relay-resolver-non-existent-type.invalid.js");
    let expected = include_str!("to_schema/fixtures/terse-relay-resolver-non-existent-type.invalid.expected");
    test_fixture(transform_fixture, "terse-relay-resolver-non-existent-type.invalid.js", "to_schema/fixtures/terse-relay-resolver-non-existent-type.invalid.expected", input, expected);
}

#[test]
fn terse_relay_resolver_with_output_type() {
    let input = include_str!("to_schema/fixtures/terse-relay-resolver-with-output-type.js");
    let expected = include_str!("to_schema/fixtures/terse-relay-resolver-with-output-type.expected");
    test_fixture(transform_fixture, "terse-relay-resolver-with-output-type.js", "to_schema/fixtures/terse-relay-resolver-with-output-type.expected", input, expected);
}

#[test]
fn terse_relay_resolver_with_root_fragment_on_model() {
    let input = include_str!("to_schema/fixtures/terse-relay-resolver-with-root-fragment-on-model.js");
    let expected = include_str!("to_schema/fixtures/terse-relay-resolver-with-root-fragment-on-model.expected");
    test_fixture(transform_fixture, "terse-relay-resolver-with-root-fragment-on-model.js", "to_schema/fixtures/terse-relay-resolver-with-root-fragment-on-model.expected", input, expected);
}

#[test]
fn terse_resolver_duplicated_invalid() {
    let input = include_str!("to_schema/fixtures/terse-resolver-duplicated.invalid.js");
    let expected = include_str!("to_schema/fixtures/terse-resolver-duplicated.invalid.expected");
    test_fixture(transform_fixture, "terse-resolver-duplicated.invalid.js", "to_schema/fixtures/terse-resolver-duplicated.invalid.expected", input, expected);
}

#[test]
fn weak_type() {
    let input = include_str!("to_schema/fixtures/weak-type.js");
    let expected = include_str!("to_schema/fixtures/weak-type.expected");
    test_fixture(transform_fixture, "weak-type.js", "to_schema/fixtures/weak-type.expected", input, expected);
}
