# Pactest
A tool for arch linux written in rust that shows all locally downloaded packages that are present in the testing respositories.
This tool does similiary to what signoff -i does.

Usage is:
```
pactest --db <repository>
```

\<repository> can be:

`core-testing`,
`extra-testing`,
`multilib-testing`,
`all`,
