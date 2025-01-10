use solana_idlgen::idlgen; idlgen!(
    {
        "metadata": { "address": "ADcaide4vBtKuyZQqdU689YqEGMCmS4tL35bdTv9wJa" },
        "version": "0.1.0",
        "name": "wba_prereq",
        "instructions": [
          {
            "name": "complete",
            "accounts": [
              {
                "name": "signer",
                "isMut": true,
                "isSigner": true
              },
              {
                "name": "prereq",
                "isMut": true,
                "isSigner": false
              },
              {
                "name": "systemProgram",
                "isMut": false,
                "isSigner": false
              }
            ],
            "args": [
              {
                "name": "github",
                "type": "bytes"
              }
            ]
          },
          {
            "name": "update",
            "descriminator": [
              219,
              200,
              88,
              176,
              158,
              63,
              253,
              127
            ],
            "accounts": [
              {
                "name": "signer",
                "isMut": true,
                "isSigner": true
              },
              {
                "name": "prereq",
                "isMut": true,
                "isSigner": false
              },
              {
                "name": "systemProgram",
                "isMut": false,
                "isSigner": false
              }
            ],
            "args": [
              {
                "name": "github",
                "type": "bytes"
              }
            ]
          }
        ],
        "accounts": [
          {
            "name": "PrereqAccount",
            "type": {
              "kind": "struct",
              "fields": [
                {
                  "name": "github",
                  "type": "bytes"
                },
                {
                  "name": "key",
                  "type": "publicKey"
                }
              ]
            }
          }
        ],
        "errors": [
          {
            "code": 6000,
            "name": "InvalidGithubAccount",
            "msg": "Invalid Github account"
          }
        ]
      }
);