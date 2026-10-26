<?php

it('é ü', function () {
    expect('ééééééééé')->toBeString()->toBe('x');
    expect('pest')->toBeString()->toBe('x/');
    expect('a')->toBeString()->toBe('/');
});
-----
<?php

it('é ü', function () {
    expect('ééééééééé')->toBe('x');
    expect('pest')->toBe('x/');
    expect('a')->toBe('/');
});
