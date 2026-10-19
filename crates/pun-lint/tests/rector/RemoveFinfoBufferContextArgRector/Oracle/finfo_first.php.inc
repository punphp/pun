<?php

finfo_buffer(context: $ctx, finfo: $finfo, string: $c, flags: FILEINFO_NONE);

function a(finfo $f)
{
    $f->buffer(context: $ctx, string: $c);
}

function b(finfo $f)
{
    $f->$method($c, FILEINFO_NONE, $ctx);
}
-----
<?php

finfo_buffer(finfo: $finfo, string: $c, flags: FILEINFO_NONE);

function a(finfo $f)
{
    $f->buffer(string: $c);
}

function b(finfo $f)
{
    $f->$method($c, FILEINFO_NONE, $ctx);
}
