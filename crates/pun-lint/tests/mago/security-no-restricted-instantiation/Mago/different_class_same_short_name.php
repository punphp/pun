<?php

namespace App;

class Client {}

// This should NOT be flagged (different class than HttpService\Client)
$myClient = new Client();
