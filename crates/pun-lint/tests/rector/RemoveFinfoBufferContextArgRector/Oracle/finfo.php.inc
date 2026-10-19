<?php

finfo_buffer($finfo);
finfo_buffer($finfo, $c, FILEINFO_NONE, $ctx);
\finfo_buffer($finfo, $c, FILEINFO_NONE, $ctx);
finfo_buffer($finfo, $c, flags: FILEINFO_NONE);

function a()
{
    $f = new finfo();
    $f->buffer($c, FILEINFO_NONE, $ctx);
    $f->buffer($c);
}

function b(\finfo $f)
{
    $f->BUFFER($c, FILEINFO_NONE, $ctx);
    (new finfo())->buffer($c, FILEINFO_NONE, $ctx);
}

function c($f)
{
    $f->buffer($c, FILEINFO_NONE, $ctx);
}
-----
<?php

finfo_buffer($finfo);
finfo_buffer($finfo, $c, FILEINFO_NONE);
\finfo_buffer($finfo, $c, FILEINFO_NONE);
finfo_buffer($finfo, $c, flags: FILEINFO_NONE);

function a()
{
    $f = new finfo();
    $f->buffer($c, FILEINFO_NONE);
    $f->buffer($c);
}

function b(\finfo $f)
{
    $f->BUFFER($c, FILEINFO_NONE);
    (new finfo())->buffer($c, FILEINFO_NONE);
}

function c($f)
{
    $f->buffer($c, FILEINFO_NONE, $ctx);
}
