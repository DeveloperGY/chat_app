let client_id = -1;

async function main() {
    try {
        await join();
        setInterval(receive, 200);
    } catch (error) {
        console.log(error);
        return;
    }
}

async function join() {
    let response = await fetch("/join", {method: "POST"});

    if (!response.ok) {
        throw Error("Failed to join chat!");
    }

    let data = await response.json();

    client_id = data.id;
    document.getElementById("client-id").innerText = client_id;
}

async function send() {
    if (client_id == -1) {
        return;
    }

    await fetch("/send", {
        method: "POST",
        body: `{"id": ${client_id}, "content": "${document.getElementById("message").value}"}`
    });
}

async function receive() {
    if (client_id == -1) {
        return -1;
    }

    let response = await fetch("/read", {
        method: "POST",
        body: `{"id": ${client_id}}`
    });

    let data = await response.json();
    
    for (let i=0; i<data.messages.length; i++) {
        let message = data.messages[i];
        let chat = document.getElementById("chat");

        let message_elem = document.createElement("p");
        message_elem.innerHTML = `${message.id}: ${message.content}`;

        chat.appendChild(message_elem);
    }
}

async function leave() {
    if (client_id != -1) {
        await fetch("/leave", {method: "POST", body: `{"id": ${client_id}}`});
        client_id = -1;
    }
}

window.onload = async () => {
    document.getElementById("message").addEventListener("keyup", async (e) => {
        if (e.key == "Enter") {
            await send();
            document.getElementById("message").value = "";
        }
    });
    await main();
}

window.onbeforeunload = async () => {
    await leave();
}

window.onclose = async () => {
    await leave();
}

window.onunload = async () => {
    await leave();
}