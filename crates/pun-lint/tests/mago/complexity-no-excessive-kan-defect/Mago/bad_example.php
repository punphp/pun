<?php

function handleRequest($request) {
    if (empty($request)) {
        return null;
    }

    if (!isset($request['type'])) {
        return null;
    }

    switch ($request['type']) {
        case 'create':
            if (!isset($request['data'])) {
                return null;
            }
            break;
        case 'update':
            if (!isset($request['id'])) {
                return null;
            }
            break;
        case 'delete':
            if (!isset($request['id'])) {
                return null;
            }
            break;
    }

    if (isset($request['filters'])) {
        foreach ($request['filters'] as $key => $value) {
            switch ($key) {
                case 'status':
                    if ($value === 'active') {
                        // filter
                    }
                    break;
                case 'category':
                    if (!empty($value)) {
                        // filter
                    }
                    break;
            }
        }
    }

    while (!empty($request['items'])) {
        $item = array_shift($request['items']);
        if ($item['valid']) {
            foreach ($item['tags'] as $tag) {
                if ($tag === 'important') {
                    // process
                }
            }
        }
    }

    foreach ($request['metadata'] as $meta) {
        switch ($meta['type']) {
            case 'timestamp':
                break;
            case 'user':
                break;
        }
    }

    return ['status' => 'success'];
}
