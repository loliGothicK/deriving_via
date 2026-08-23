# Deriving Via: Trait Reference

This is a comprehensive reference document summarizing the behavior of all traits supported by the `deriving_via` macro when they are derived.
Each section explains what kind of code is generated **without the `via` attribute** and **with the `via` attribute**, as well as the trait bounds (e.g., `where` clauses) and requirements placed on the underlying or `via` types.
