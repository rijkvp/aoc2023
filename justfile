alias c := comp
alias r := run
alias e := test-example
alias t := test

tmpdir := `mktemp -d`

# compile a Rust file
[no-cd]
comp file:
    rustc {{file}} -C opt-level=3 -o {{tmpdir}}/{{file_name(file)}}

# run a rust file
[no-cd]
run file: (comp file)
    {{tmpdir}}/{{file_name(file)}}

# test a rust file with input
[no-cd]
test-example file: (comp file)
    {{tmpdir}}/{{file_name(file)}} < example

# test a rust file with input
[no-cd]
test file: (comp file)
    {{tmpdir}}/{{file_name(file)}} < input
