<?php

namespace App;

app();
app(abstract: 'foo');
app(parameters: [], abstract: 'foo');
app(parameters: []);
app(...$args);
\app('foo');
$service = null;
app($service);
app(NULL, []);
Container::app('foo');
-----
<?php

namespace App;

app();
resolve(abstract: 'foo');
resolve(parameters: [], abstract: 'foo');
app(parameters: []);
app(...$args);
resolve('foo');
$service = null;
app($service);
app(NULL, []);
Container::app('foo');
