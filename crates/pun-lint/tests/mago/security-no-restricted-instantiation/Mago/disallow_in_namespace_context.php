<?php

namespace App\Services;

use HttpService\Client;

class MyService {
    public function create() {
        return new Client();
    }
}
