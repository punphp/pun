<?php

namespace N;

use function SomeNamespaced\view as nsview;

class C
{
    public function run()
    {
        view('template', []);
        \VIEW('t');
        nsview('t');
        \SomeNamespaced\view();
        $a = view(
            'template', // the template
            []
        );
        $g = view(...$args)->render();
    }
}
-----
<?php

namespace N;

use function SomeNamespaced\view as nsview;

class C
{
    public function run()
    {
        \SomeStaticClass::render('template', []);
        \SomeStaticClass::render('t');
        \AnotherStaticClass::render('t');
        \AnotherStaticClass::render();
        $a = \SomeStaticClass::render(
            'template',
            // the template
            []
        );
        $g = \SomeStaticClass::render(...$args)->render();
    }
}
