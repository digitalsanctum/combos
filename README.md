# combos

Given a file and integer k, print all possible permutations concatenated.

## usage

Given the file `/path/to/test` with the following contents:

```shell
foo
bar
zoo
```

The command:
```shell
combos /path/to/test 2
```

will result in:

```shell
foobar
foozoo
barfoo
barzoo
zoofoo
zoobar
```