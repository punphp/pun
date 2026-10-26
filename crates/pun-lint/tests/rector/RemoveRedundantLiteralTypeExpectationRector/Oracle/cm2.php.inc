<?php

it('a', function () {
    $x = /* c */ expect('a')->toBeString()->toBe('a');
    $x = /* c */ expect('a')->toBeString()->toBe('a')->toBe('a');
    foo(/* c */ expect('a')->toBeString()->toBe('a'));
    foo(/* c */ expect('a')->toBeString()->toBe('a')->toBe('a'));
    return /* c */ expect('a')->toBeString()->toBe('a');
});
-----
<?php

it('a', function () {
    $x = /* c */ expect('a')->toBeString()->toBe('a');
    $x = /* c */ expect('a')->toBe('a')->toBe('a');
    foo(/* c */ expect('a')->toBe('a'));
    foo(/* c */ expect('a')->toBe('a')->toBe('a'));
    return /* c */ expect('a')->toBeString()->toBe('a');
});
