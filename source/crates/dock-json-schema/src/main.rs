mod wire;

fn main() -> Result<(), wire::ProcessFailure> {
    wire::run_one_shot()
}
