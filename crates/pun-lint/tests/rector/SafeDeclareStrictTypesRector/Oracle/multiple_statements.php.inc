<?php

// a comment
function pun_oracle_safe_strict_f(): void
{
}

pun_oracle_safe_strict_f();
echo 1;
-----
<?php

declare(strict_types=1);

// a comment
function pun_oracle_safe_strict_f(): void
{
}

pun_oracle_safe_strict_f();
echo 1;
