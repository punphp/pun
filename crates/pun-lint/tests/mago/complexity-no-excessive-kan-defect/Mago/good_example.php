<?php

function handleRequest($request) {
    $validated = validateRequest($request);
    $processed = processRequest($validated);
    return formatResponse($processed);
}

function validateRequest($request) {
    if (empty($request['type'])) {
        return null;
    }
    return $request;
}

function processRequest($request) {
    return match($request['type']) {
        'create' => createResource($request),
        'update' => updateResource($request),
        'delete' => deleteResource($request),
        default => null
    };
}

function formatResponse($data) {
    return ['status' => 'success', 'data' => $data];
}
