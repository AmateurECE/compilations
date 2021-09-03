///////////////////////////////////////////////////////////////////////////////
// NAME:            index.js
//
// AUTHOR:          Ethan D. Twardy <edtwardy@mtu.edu>
//
// DESCRIPTION:     Main script for the page.
//
// CREATED:         07/26/2021
//
// LAST EDITED:     08/28/2021
////

let videoList; // Global state!?
let videoBoxOne;
let videoBoxTwo;

function getMeta(metaName) {
    const metas = document.getElementsByTagName('meta');

    for (let i = 0; i < metas.length; i++) {
        if (metas[i].getAttribute('name') === metaName) {
            return metas[i].getAttribute('content');
        }
    }

    return '';
}

class ApiClient {
    constructor(rootUrl) {
        this.rootUrl = rootUrl;
        this.csrfmiddlewaretoken = document.querySelector(
            '[name=csrfmiddlewaretoken]').value;
    }

    async jsonFetch(endpoint) {
        const response = await fetch(
            this.rootUrl + endpoint,
            {headers: {'Content-Type': 'application/json'}}
        );

        if (!response.ok) {
            throw Error(response.statusText);
        }

        return response.json();
    }

    async jsonDelete(endpoint) {
        const response = await fetch(
            this.rootUrl + endpoint, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    'X-CSRFToken': this.csrfmiddlewaretoken,
                }
            }
        );

        if (!response.ok) {
            throw Error(response.statusText);
        }

        return {};
    }
}

async function kickoffVideoLoop() {
    const boxElement = document.getElementById(this.elementId);
    if (boxElement.firstElementChild !== null) {
        const videoElement = boxElement.firstChild;
        boxElement.removeChild(videoElement);
    }
    if (this.videoBox !== undefined) {
        await this.client.jsonDelete('videos/' + this.videoBox.name + '/');
    }

    const videoElement = document.createElement('video');
    videoElement.setAttribute('controls', '');
    boxElement.appendChild(videoElement);

    let video = videoList.videos.shift();
    let sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));
    if (video === undefined) {
        while (true) {
            if (videoList.after !== null) {
                videoList = await this.client.jsonFetch(
                    'videos/' + `?after=${videoList.after}`
                        + `&count=${videoList.count}`);
                console.log(videoList);
                if (videoList.videos.length === 0) {
                    await sleep(2000);
                } else {
                    video = videoList.videos.shift();
                    break;
                }
            } else {
                break;
            }
        }
    }

    if (video !== undefined) {
        this.videoBox = video;
        const url = await this.client.jsonFetch('videos/' + video.guid + '/');
        videoElement.src = url;
        console.log(videoElement.src);
        videoElement.addEventListener(
            'ended', kickoffVideoLoop.bind(this), false);
        videoElement.addEventListener(
            'canplaythrough', async () => { await videoElement.play(); });
    }
}

async function main() {
    const button = document.getElementById('start');
    button.parentNode.removeChild(button);

    const box1 = document.createElement('div');
    box1.classList.add('video-box');
    box1.id = 'box-1';
    const box2 = document.createElement('div');
    box2.classList.add('video-box');
    box2.id = 'box-2';
    const videoPlayer = document.getElementById('player');
    videoPlayer.appendChild(box1);
    videoPlayer.appendChild(box2);

    const apiUrl = getMeta('api-base');
    const client = new ApiClient(apiUrl);
    videoList = await client.jsonFetch('videos/');

    const loop1 = kickoffVideoLoop.bind({
        client, elementId: 'box-1', videoBox: videoBoxOne});
    await loop1();

    const loop2 = kickoffVideoLoop.bind({
        client, elementId: 'box-2', videoBox: videoBoxTwo});
    await loop2();
}

window.addEventListener('DOMContentLoaded', () => {
    const button = document.createElement('button');
    button.id = 'start';
    button.addEventListener('click', main, false);
    button.appendChild(document.createTextNode('Start'));
    document.body.appendChild(button);
    // main().catch(error => console.error(error));
});

///////////////////////////////////////////////////////////////////////////////
