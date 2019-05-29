# AWS Push Code

Grab code from a remote Git repo and put it into S3. Designed as a
webhook for GitLab but easy to add additional functions to handle
webhook data from other systems.

## GitLab Integration

If your GitLab version doesn't have API key support for webhooks,
add a query parameter, _apiKey_, with the value of the API key you
want to use.

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


