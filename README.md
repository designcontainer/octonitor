# Introduction
Hello, this is the documentation for the octonitor project.

The goal of the project is to monitor the domains over at [wpengine](https://wpengine.com) and send a notification to a slack channel when a domain is down. That is a very bare bone description.

List over functionality:
- Monitor the domains
- Send notification when something is down
- Option to ignore domains
- Can run in a containerized environment

# Code what and how
Description of the files:

## Rust files

`endpoint.rs` - This file contains the structs for the Slack api endpoints. This was to make serialization (turning into json) easier, but it is kinda akward. Oh yeah and it's also so I don't have to manually write the method every time, but idk if the abstraction is really necessary here.

`message.rs`  - This is a structure which mimics the information in the slack message. This is also to make the serialization easier and have to not have to hardcode some blahblahblah.

`bot.rs`      - Actual bot code that sends the messages and stuff. Kinda self explanitory just a little worse than plain English.

## Documentation files
`something.excalidraw` - This is a diagram of the monitor tool made on the [excalidraw](https://excalidraw) so somewhere on the site there is probably an option to import files or just drag 'n drop the file over.

`gaming.json` - The json response returned from the server. Good if you need to debug or rewrite my shitty code. So atleast in the year of our lord 2025 that is what the response looked like.

## Misc files
`to.ignore`   - This is the file that contains the domains to ignore. It should be placed into a persistent volume when ran in a containerized environtment. Otherwise just make it accesible to the binary, to find the place where it is loaded in the rs shit do `grep -rnH --exclude-dir=target "to.ignore" ./*` from the project root where this file is.

`.secrets`    - Very important see the 1Password secret note named "Octonitor secrets" it contains what you need to paste into ".secrets" before you run the project.
# Dockershit
Most of the file is pretty self explanitory. To be general the Dockerfile does the following:
- Get the Rust image
- Use Rust image to build our executable
- Download the debian
- Update the package list and download a lib. Then delete the package list to save space
- Copy our executable from the builder (Rust image) to the current image
- Then get our secrets
- Use the secrets file to set ENV variables. Write a kilobyte of 0s to the secrets file. Delete the secrets file.
- Run our application

# Run script
`bash` is just another way to call on the `bash` interpreter. Could have done `./<script>`, but then the script would need to be executable and I can't be bothered.

Makes it a little easier to run containerize commands. Contains a `run` and `build` option. `run` is more for debugging when shit does not work. Examples:
```sh
bash run.sh podman build
```
Here we see an example where we use the script to build the container image. Pretty cool. You can also run:
```sh
bash run.sh podman run
```
This will put you into a interactive shell.
=======
# Octonitor monitoring bot

For monitoring the status of wordpress installations. It sends messages on Slack, cool.
