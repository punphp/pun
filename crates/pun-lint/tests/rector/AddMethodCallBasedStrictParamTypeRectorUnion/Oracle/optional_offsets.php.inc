<?php

namespace Probe;

final class Shapes
{
    public function run(string $path)
    {
        $shape = pathinfo($path);
        $this->dirname($shape['dirname']);
        $this->extension($shape['extension']);
        $empty = pathinfo('');
        $this->basename($empty['basename']);
        $this->emptyDirname($empty['dirname']);
    }

    private function dirname($param)
    {
    }

    private function extension($param)
    {
    }

    private function basename($param)
    {
    }

    private function emptyDirname($param)
    {
    }
}
-----
<?php

namespace Probe;

final class Shapes
{
    public function run(string $path)
    {
        $shape = pathinfo($path);
        $this->dirname($shape['dirname']);
        $this->extension($shape['extension']);
        $empty = pathinfo('');
        $this->basename($empty['basename']);
        $this->emptyDirname($empty['dirname']);
    }

    private function dirname($param)
    {
    }

    private function extension($param)
    {
    }

    private function basename(string $param)
    {
    }

    private function emptyDirname($param)
    {
    }
}
