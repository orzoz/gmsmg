Suggest a precise and commit message based on the following diff. The order is not important. Any code modifications should be given attention

<MUST>SUBJECT SHOULD BE AS SHORT AS POSSIBLE</MUST>
<MUST>SUBJECT SHOULD LESS THAN 50 CHARACTERS</MUST>
<MUST>SCOPE SHOULD BE A LOWERCASE WORD</MUST>
<MUST>MUST USE ABBREVIATIONS</MUST>

- <type> is one of: build, chore, ci, docs, feat, fix, perf, refactor, revert, style, test
- <scope> is optional, use a lowercase word to represent the affected module (e.g., core, common, forms)
- <subject> starts with lowercase, brevity, doesn't end with a period
- <body> is optional, uses present tense, use markdown syntax body if needed, and wraps at 72 characters

The commit message should follow the json format or it will be rejected:

{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "properties": {
        "type": {
            "type": "string"
        },
        "scope": {
            "type": "string"
        },
        "subject": {
            "type": "string",
            "maxLength": 50
        },
        "body": {
            "type": "string"
        }
    },
    "additionalProperties": false,
    "required": [
        "type",
        "subject"
    ]
}

Example1: {
    "type": "fix",
    "scope": "auth",
    "subject": "handle 403 forbidden error cases",
}

Example2: {
    "type": "feat",
    "subject": "add ability to update user avatar",
    "body": "Implement a new feature allowing users to upload and update their profile avatar. \nThis change includes:\n- New API endpoint for avatar upload\n- Frontend UI updates in the profile section\n- Image processing to resize and optimize uploaded avatars"
}

Diff: