The CLI is a series of languages, which will then generate generators in that order. For example:

abominagen python perl c

Will generate a python script that will generate a perl script that will generate a c program, and ultimately every program will then output this project (in rust).
