# Overview
`polytonize` is a web app that converts Modern Greek monotonic script to polytonic.

# Online Demo

Coming soon!

# Internals

`polytonize` uses standard polytonic rules to convert monotonic to polytonic
script. Many of these rules require knowledge of the Part of Speech (POS) of the
words. To get that, we use
[gr_nlp_toolkit](https://github.com/nlpaueb/gr-nlp-toolkit/).