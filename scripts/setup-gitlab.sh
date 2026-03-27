#!/usr/bin/env bash

git remote set-url --push --add origin "$(git remote get-url --push origin)"
git remote set-url --push --add origin https://gitlab-stud.elka.pw.edu.pl/ddsouza/zpr-przeszukiwanie-skanow
