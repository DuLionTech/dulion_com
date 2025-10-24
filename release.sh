#!/bin/bash

trunk build --release
aws s3 sync ./dist/ s3://www.dulion.com/ --delete