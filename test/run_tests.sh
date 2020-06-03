#!./libs/bats/bin/bats
load 'libs/bats-support/load'
load 'libs/bats-assert/load'
load 'load'

@test "create new server env" {
  run alienv new server
  run alienv show
  assert_output --partial 'server*'
}

alienv new server
alienv add hw 'echo "Hello, World"'

@test "add alias" {
  #setup rm -rf ~/.alienv/envs
#  run alienv new server
 # run alienv add hw "echo 'Hello, World!'"
  run hw
  assert_output 'Hello, World!'
}
