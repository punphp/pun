<?php

it('a', function () {
    expect('-1.5')->toBeNumeric()->toBe('-1.5');
    expect('+.5')->toBeNumeric()->toBe('+.5');
    expect('1e+5')->toBeNumeric()->toBe('1e+5');
    expect('1E-5')->toBeNumeric()->toBe('1E-5');
    expect('1e')->toBeNumeric()->toBe('1e');
    expect('1.')->toBeNumeric()->toBe('1.');
    expect('.')->toBeNumeric()->toBe('.');
    expect(' ')->toBeNumeric()->toBe(' ');
    expect(<<<EOT
        a {$x}
        EOT)->toBeString()->toBe('a');
    expect(<<<EOT
        12
        EOT)->toBeNumeric()->toBe('12');
});
-----
<?php

it('a', function () {
    expect('-1.5')->toBe('-1.5');
    expect('+.5')->toBe('+.5');
    expect('1e+5')->toBe('1e+5');
    expect('1E-5')->toBe('1E-5');
    expect('1e')->toBeNumeric()->toBe('1e');
    expect('1.')->toBe('1.');
    expect('.')->toBeNumeric()->toBe('.');
    expect(' ')->toBeNumeric()->toBe(' ');
    expect(<<<EOT
        a {$x}
        EOT)->toBeString()->toBe('a');
    expect(<<<EOT
        12
        EOT)->toBe('12');
});
