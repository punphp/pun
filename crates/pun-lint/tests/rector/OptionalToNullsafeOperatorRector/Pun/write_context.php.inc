<?php
// A nullsafe property is not writable. Rector generates PHP that does not parse here.
optional($value)->property = 1;
optional($value)->property++;
