<?php

it('a', function () {
    // before the statement
    expect('pest')
        ->toBeString()
        ->toStartWith('p');

    expect('http://pest')
        ->toBeString()
        ->toStartWith('h');

    expect(/* inside */ 'pest')->toBeString()->toStartWith('p');

    expect('pest')->toBeString()->toStartWith('p') /* after */;

    $x = expect('pest')->toBeString()->toStartWith('p'); // trailing

    expect('pest')->toBeString()->toStartWith('p')->toEndWith('t'); // trailing
});
-----
<?php

it('a', function () {
    // before the statement
    expect('pest')
        ->toStartWith('p');

    expect('http://pest')
        ->toBeString()
        ->toStartWith('h');

    expect(/* inside */ 'pest')->toBeString()->toStartWith('p');

    expect('pest')->toStartWith('p') /* after */;

    $x = expect('pest')->toStartWith('p'); // trailing

    expect('pest')->toStartWith('p')->toEndWith('t'); // trailing
});
