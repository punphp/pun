<?php

interface Htmlable
{
    public function toHtml();
}

class Bag implements Htmlable, \Stringable
{
    public function __toString()
    {
        return 'x';
    }

    public function toHtml()
    {
        return (string) $this;
    }
}
-----
<?php

interface Htmlable
{
    public function toHtml();
}

class Bag implements Htmlable, \Stringable
{
    public function __toString()
    {
        return 'x';
    }

    public function toHtml(): string
    {
        return (string) $this;
    }
}
