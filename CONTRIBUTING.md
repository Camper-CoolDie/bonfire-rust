# Contributing to bonfire-rust

This guide outlines best practices for contributing to the project.

## Code Style

Consistency in code style is important for readability and maintainability. Please adhere to the
following:

1. Use clear and descriptive names for variables.

   :x: **Avoid**:
   ```rust
   for (i, v) in vec.into_iter().enumerate() {
       res.push(v.inspect_err(
           |e| tracing::error!(?e, "failed to parse value")
       )?);
   }
   ```

   :white_check_mark: **Instead, do**:
   ```rust
   for (index, value) in vector.into_iter().enumerate() {
       result.push(value.inspect_err(
           |error| tracing::error!(?error, "failed to parse value")
       )?);
   }
   ```

2. Always run `cargo +nightly fmt`, `cargo clippy` and `cargo test` before submitting a pull
   request. The CI pipeline will enforce these checks.

## Documentation Guidelines

Clear and consistent documentation is vital. Please follow these rules:

1. Documentation strings (for structs, enums, traits, functions) should **end with a dot**, except
   for:
   - Struct fields
   - Enum variants
   - Fields within enum struct variants

2. Functions that return a `Result` type **must** include an `# Errors` section detailing all
   possible errors, including the generic `crate::Error` catch-all.

   - **For 2 errors**: Separate them with `or`.
     ```rust
     /// # Errors
     ///
     /// Returns [`SetAgeError::InvalidAge`] if the provided age is outside the allowed range, or
     /// [`Error`][crate::Error] if any other error occurs during the request.
     ```

   - **For 3 or more errors**: Use bullet points (`*`).
     ```rust
     /// # Errors
     ///
     /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the authenticated
     ///   user's access level is below the requirement.
     /// * Returns [`SetProfileTextError::TooLong`] if the provided status exceeds the maximum
     ///   allowed length.
     /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
     ```

3. Clearly state the authentication and access level requirements for each method in its main
   documentation block.

   ```rust
   /// This method does not require authentication.
   ```

   ```rust
   /// This operation requires an authenticated client with at least
   /// [`AccessLevel::Experienced`][crate::models::AccessLevel::Experienced] permissions.
   ```

4. Functions that return `impl Stream<Item = Result<...>>` should list all possible errors directly
   within the main documentation block, *without* a separate `# Errors` section.

   ```rust
   /// If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream will yield
   /// that single error and then terminate.
   ```

5. Always use backticks for cross-references to code items (e.g.,
   ``[`Client::login`][crate::Client::login]``).

6. Before writing any new documentation, please review existing documentation strings in similar
   contexts. Maintaining a consistent style greatly improves readability.

---

Thank you for contributing to `bonfire-rust`!
