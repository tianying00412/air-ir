use air_ir::Air;

use super::Scope;

/// Updates the provided scope with a public inputs.
pub(super) fn add_public_inputs_struct(scope: &mut Scope, ir: &Air) {
    let name = "PublicInputs";
    // define the PublicInputs struct.
    let pub_inputs_struct = scope.new_struct(name).vis("pub");

    for public_input in ir.public_inputs() {
        pub_inputs_struct.field(
            public_input.name.as_str(),
            format!("[Felt; {}]", public_input.size),
        );
    }

    // add the public inputs implementation block
    let base_impl = scope.new_impl(name);

    let pub_inputs_values: Vec<String> = ir
        .public_inputs()
        .map(|input| input.name.to_string())
        .collect();

    // add a constructor for public inputs
    let new_fn = base_impl
        .new_fn("new")
        .vis("pub")
        .ret("Self")
        .line(format!("Self {{ {} }}", pub_inputs_values.join(", ")));
    for public_input in ir.public_inputs() {
        new_fn.arg(
            public_input.name.as_str(),
            format!("[Felt; {}]", public_input.size),
        );
    }

    add_serializable_impl(scope, pub_inputs_values)
}

/// Adds Serialization implementation for PublicInputs to the scope
fn add_serializable_impl(scope: &mut Scope, pub_input_values: Vec<String>) {
    let serializable_impl = scope.new_impl("PublicInputs").impl_trait("Serializable");
    let write_into_fn = serializable_impl
        .new_fn("write_into")
        .generic("W: ByteWriter")
        .arg_ref_self()
        .arg("target", "&mut W");
    for pub_input_value in pub_input_values {
        write_into_fn.line(format!("target.write(self.{pub_input_value}.as_slice());"));
    }
}
