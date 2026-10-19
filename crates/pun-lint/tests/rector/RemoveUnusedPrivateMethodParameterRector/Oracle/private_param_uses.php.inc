<?php

final class Uses
{
    public function go()
    {
        return [$this->a(1), $this->c(1)];
    }

    private function a($x)
    {
        return compact('x');
    }

    private function c($x)
    {
        function inner($x)
        {
            return $x;
        }
        return compact('z');
    }
}
-----
<?php

final class Uses
{
    public function go()
    {
        return [$this->a(1), $this->c()];
    }

    private function a($x)
    {
        return compact('x');
    }

    private function c()
    {
        function inner($x)
        {
            return $x;
        }
        return compact('z');
    }
}
