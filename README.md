# AWS Push Code

Grab code from a remote Git repo and put it into S3. Designed as a
webhook for GitLab but easy to add additional functions to handle
webhook data from other systems.

## Status

### Features

Currently, this project is MVP for a specific CD pipeline at CJ.
With a few enhancements, it will be generally useful:

- Differentiate sources in S3. Currently each source zip object is
  stored to `master.zip`
- Add a bit more information to the source metadata (e.g. branch, commit time)
  to enable additional build logic
- Generalize the architecture. Right now it is very CJ specific; it
  relies on exported Cloud Formation outputs unique to CJ internal
  architecture.
- Do something better with failures in the architecture. They just go
  to a DLQ right now.
- Authentication


### Code

- Deepen the testing: right now, the main logic is tested but there is too
  much untested code in the effects. This is mostly due to my Rust newness.
  I've been honing my Rust and figuring out the best ways for transitive
  testing so this should be done in the next few revisions.
- Make the code a little more OO. Current code is structured similarly to my
  Haskell code. I'd like to introduce some Object orientation back into it.
  Still establishing my Rust design sense so this should get better over time.

## Deploy

Deploy with Serverless Framework: `npx sls deploy`.

## How it Works

### Lambda Functions

There are two Lambda functions: _accept_ and _work_.
- _accept_: respond to a webhook. This function converts the webhook
    event into a standard work event and places it onto an SQS queue.
    This is a simple conversion operation. If you need additional webhook
    integrations for other products, model them after this.
- _work_: perform the code push work. This accepts standardized work
    events from the queue and performs code pushes as specified by them.

The system uses multiple Lambda functions for two reasons:
- separate webhook interpretation from work. This makes it easy to
  specify additional webhook integration types.
- meet webhook timeout deadlines. In the case of GitLab, the webhook
  must respond within 10 seconds. In this architecture, the _accept_
  function only needs to enqueue the work so can easily meet the
  deadline. The _work_ function can then take as long as it needs
  to process the git repository and upload it.

### API Gateway

A POST method via API Gateway invokes the _accept_ Lambda function.
Authorizers should be integrated with the Gateway, authentication
should not be implemented in _accept_ handlers.

### Smoke Testing

Run from the command line with `cargo run --bin main <REPO> <BUCKET> <KEY>`
to upload the contents of REPO to BUCKET under key, KEY; e.g.
`cargo run --bin main 'git@github.com:cj-greenhouse/aws-push-code.git' pipelinesources-19203492 master.zip`

If you receive an S3 redirection error, set the AWS_REGION environment variable
to where your bucket was created. This seems to be a bug in the rusoto
library.

If the repo is private, put a suitable private SSH key into secrets manager
and set the CJ_PUSHCODE_GIT_CREDENTIALS_ID environment to the ID of the
secret.

