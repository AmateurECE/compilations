///////////////////////////////////////////////////////////////////////////////
// NAME:            index.js
//
// AUTHOR:          Ethan D. Twardy <edtwardy@mtu.edu>
//
// DESCRIPTION:     Main script for the page.
//
// CREATED:         07/26/2021
//
// LAST EDITED:     07/26/2021
////

import SwaggerClient from 'swagger-client';
const openapiUrl = "/compilations/api/";

let videoList; // Global state!?

async function kickoffVideoLoop() {
    const boxElement = document.getElementById(this.elementId);
    if (boxElement.firstElementChild !== null) {
        const videoElement = boxElement.firstChild;
        boxElement.removeChild(videoElement);
    }

    const videoElement = document.createElement('video');
    videoElement.setAttribute('controls', '');
    boxElement.appendChild(videoElement);

    const newVideo = videoList.shift();
    if (newVideo !== undefined) {
        const videoUrl = await this.client.execute({
            operationId: 'retrieveVideoCollection',
            parameters: {
                'id': newVideo.id,
            }});
        videoElement.src = videoUrl.body;
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

    const client = await SwaggerClient({url: openapiUrl});
    const data = await client.execute({operationId: "listVideoCollections"});
    videoList = data.body;

    const loop1 = kickoffVideoLoop.bind({client, elementId: 'box-1'});
    await loop1();

    const loop2 = kickoffVideoLoop.bind({client, elementId: 'box-2'});
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
