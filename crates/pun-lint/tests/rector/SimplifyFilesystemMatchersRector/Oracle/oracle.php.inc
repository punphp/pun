<?php

test('a', function ($p) {
    expect(is_file($p) && is_readable($p))->toBeTrue();
    expect(is_dir($p) && is_writeable($p))->toBeTrue();
    expect(is_file($p) and is_readable($p))->toBeTrue();
    expect(is_file($p) && is_readable($p))->not->toBeTrue();
    expect(is_file($p) && is_readable($p))->toBeTrue('msg');
    expect(is_file($p) && is_readable($p))
        ->toBeTrue();
    expect(is_file($p) && is_readable($p))
        ->toBeTrue('msg');
    expect(is_file($p) && is_readable($p))->toBeString()->toBeTrue();
    expect(is_file($p) && is_readable($p))->TOBETRUE();
    expect(IS_FILE($p) && is_readable($p))->toBeTrue();
    expect(\is_file($p) && \is_readable($p))->toBeTrue();
    expect(is_file($p) && is_file($p))->toBeTrue();
    expect(is_file($p, 1) && is_readable($p))->toBeTrue();
    expect(is_file($a->b) && is_readable($a -> b))->toBeTrue();
    expect($x)->and(is_file($p) && is_readable($p))->toBeTrue();
    expect(is_file($p) && is_readable($p), 'x')->toBeTrue();
    expect(is_dir($p) && is_readable($p) && is_writable($p))->toBeTrue();
    expect($p)->toBeFile()->toBeReadable();
    expect($p)->toBeDirectory()->toBeWritable();
    expect($p)->toBeFile()->TOBEREADABLE();
    expect($p)->TOBEFILE()->toBeReadable();
    expect($p)->toBeFile('x')->toBeReadable();
    expect($p)->not->toBeFile()->toBeReadable();
    expect($p)->toBeFile()->not->toBeReadable();
    expect($p)
        ->toBeFile()
        ->toBeReadable();
    expect($p)->toBeReadable()->toBeFile();
    expect($p)->toBeFile()->toBeReadable()->toBeWritable();
    expect($p)->toBeString()->toBeDirectory()->toBeReadable();
});
-----
<?php

test('a', function ($p) {
    expect($p)->toBeReadableFile();
    expect($p)->toBeWritableDirectory();
    expect(is_file($p) and is_readable($p))->toBeTrue();
    expect($p)->toBeReadableFile();
    expect($p)->toBeReadableFile();
    expect($p)
        ->toBeReadableFile();
    expect($p)->toBeReadableFile();
    expect($p)->toBeReadableFile();
    expect($p)->toBeReadableFile();
    expect(IS_FILE($p) && is_readable($p))->toBeTrue();
    expect($p)->toBeReadableFile();
    expect(is_file($p) && is_file($p))->toBeTrue();
    expect(is_file($p, 1) && is_readable($p))->toBeTrue();
    expect($a->b)->toBeReadableFile();
    expect($x)->and($p)->toBeReadableFile();
    expect($p)->toBeReadableFile();
    expect(is_dir($p) && is_readable($p) && is_writable($p))->toBeTrue();
    expect($p)->toBeReadableFile();
    expect($p)->toBeWritableDirectory();
    expect($p)->toBeFile()->TOBEREADABLE();
    expect($p)->TOBEFILE()->toBeReadable();
    expect($p)->toBeFile('x')->toBeReadable();
    expect($p)->not->toBeReadableFile();
    expect($p)->toBeFile()->not->toBeReadable();
    expect($p)
        ->toBeReadableFile();
    expect($p)->toBeReadable()->toBeFile();
    expect($p)->toBeReadableFile()->toBeWritable();
    expect($p)->toBeString()->toBeReadableDirectory();
});
