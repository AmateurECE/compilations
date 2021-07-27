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

window.addEventListener('DOMContentLoaded', () => {
    SwaggerClient({url: openapiUrl})
        .then(client => client.execute({
            operationId: "listVideoCollections",
        }))
        .then(data => document.body.innerHTML = JSON.stringify(data.body))
        .catch(error => console.error(error));
});

///////////////////////////////////////////////////////////////////////////////
