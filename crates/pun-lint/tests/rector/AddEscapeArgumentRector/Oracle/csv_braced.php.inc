<?php

namespace {
    use SplFileObject as Csv;

    function a(Csv $file)
    {
        $file->fputcsv([1]);
    }

    function b()
    {
        $file = new namespace\SplFileObject('x');
        $file->fputcsv([1]);
    }
}
-----
<?php

namespace {
    use SplFileObject as Csv;

    function a(Csv $file)
    {
        $file->fputcsv([1], escape: '\\');
    }

    function b()
    {
        $file = new namespace\SplFileObject('x');
        $file->fputcsv([1], escape: '\\');
    }
}
