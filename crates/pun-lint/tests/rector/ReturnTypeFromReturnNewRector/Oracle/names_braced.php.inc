<?php

namespace Pun\Names\Lib {
    class Other
    {
    }
}

namespace Pun\Names\Lib\Sub {
    class Deep
    {
    }
}

namespace Pun\Names\App {
    use Pun\Names\Lib\{Item, Other as Renamed};
    use Pun\Names\Lib as L;
    use function strlen;
    use function Pun\Names\{helperA, helperB};

    class Local
    {
    }

    final class Maker
    {
        public function item()
        {
            return new Item();
        }

        public function renamed()
        {
            return new Renamed();
        }

        public function deep()
        {
            return new L\Sub\Deep();
        }

        public function local()
        {
            return new Local();
        }
    }
}
-----
<?php

namespace Pun\Names\Lib {
    class Other
    {
    }
}

namespace Pun\Names\Lib\Sub {
    class Deep
    {
    }
}

namespace Pun\Names\App {
    use Pun\Names\Lib\{Item, Other as Renamed};
    use Pun\Names\Lib as L;
    use function strlen;
    use function Pun\Names\{helperA, helperB};

    class Local
    {
    }

    final class Maker
    {
        public function item(): \Pun\Names\Lib\Item
        {
            return new Item();
        }

        public function renamed(): \Pun\Names\Lib\Other
        {
            return new Renamed();
        }

        public function deep(): \Pun\Names\Lib\Sub\Deep
        {
            return new L\Sub\Deep();
        }

        public function local(): \Pun\Names\App\Local
        {
            return new Local();
        }
    }
}
