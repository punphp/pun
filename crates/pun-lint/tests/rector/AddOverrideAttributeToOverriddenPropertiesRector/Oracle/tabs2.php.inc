<?php

namespace M {
	class B
	{
		public $a;
	}

	class C extends B
	{
		public $a;
	}
}
-----
<?php

namespace M {
	class B
	{
		public $a;
	}

	class C extends B
	{
		#[\Override]
        public $a;
	}
}
