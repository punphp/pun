<?php

// Direct instantiation of disallowed type
$client = new HttpService\Client();

// Another disallowed instantiation
$db = new DatabaseConnection('localhost', 'user', 'pass');
