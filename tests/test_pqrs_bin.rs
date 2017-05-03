extern crate protobuf;

mod runner;

use std::process::Output;
use runner::Runner;

fn for_dog_stream_with_trail(work: &mut Runner) {
    work.cmd.arg("--stream=varint");
    work.cmd.arg("--trail=1");
    work.stdin_from_file("samples/dog_stream_trail");
}

fn for_person(work: &mut Runner) {
    work.stdin_from_file("samples/person");
}

fn for_dog(work: &mut Runner) {
    work.stdin_from_file("samples/dog");
}

fn for_nonexistent_fdset_dir(work: &mut Runner) {
    work.cmd.env("FDSET_PATH", "fdset-doesnt-exist");
    work.stdin_from_file("samples/dog");
}

fn for_no_valid_fdsets(work: &mut Runner) {
    work.cmd
        .env("FDSET_PATH", &work.tests_path.join("fdsets-invalid"));
}

fn for_bad_input(work: &mut Runner) {
    work.stdin_from_file("samples/bad");
}

fn for_dog_stream(work: &mut Runner) {
    work.cmd.arg("--stream=varint");
    work.stdin_from_file("samples/dog_stream");
}

fn run_pqrs<F>(modify_in: F) -> Output
    where F: FnOnce(&mut Runner)
{
    let mut work = Runner::new();

    work.cmd
        .env("FDSET_PATH", &work.tests_path.join("fdsets"));

    modify_in(&mut work);

    work.spawn();
    work.output()
}

#[test]
fn test_dog_decode() {
    let out = run_pqrs(for_dog);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":3,\"breed\":\"gsd\",\"temperament\":\"excited\"}");
}

#[test]
fn test_dog_decode_stream() {
    let out = run_pqrs(for_dog_stream);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":2,\"breed\":\"rottweiler\",\"temperament\":\"chill\"}");
}
#[test]
fn test_dog_decode_stream_with_trail() {
    let out = run_pqrs(for_dog_stream_with_trail);
    println!("{:?}", String::from_utf8_lossy(&out.stderr));
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"age\":2,\"breed\":\"rottweiler\",\"temperament\":\"chill\"}{\"age\":9,\"breed\":\"poodle\",\"temperament\":\"aggressive\"}");
}

#[test]
fn test_nonexistent_fdset_dir() {
    let out = run_pqrs(for_nonexistent_fdset_dir);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr)
            .contains("Path fdset-doesnt-exist doesn\'t exist"));
}

#[test]
fn test_no_fdset_files() {
    let out = run_pqrs(for_no_valid_fdsets);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr).contains("No valid fdset files in path"));
}

#[test]
fn test_person_decode() {
    let out = run_pqrs(for_person);
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout),
               "{\"id\":0,\"name\":\"khosrov\"}");
}

#[test]
fn test_bad_input() {
    let out = run_pqrs(for_bad_input);
    assert_eq!(out.status.code().unwrap(), 255);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(String::from_utf8_lossy(&out.stderr)
            .contains("Couldn\'t decode with any message descriptor\n"));
}
