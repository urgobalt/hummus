use api_request::DefaultBackend;
use api_request::note::{NOteID, Note, NoteSearch, NoteUpdate};
// --- The Macro Definition (with optional improvement) ---
macro_rules! specialize_functions {
    (
        $namespace:path,
        $GenericType:ty;
        $( // Repetition for each function block {}
            {
                $func_name:ident,
                // Match argument list tuple, allowing optional trailing comma inside
                ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ),
                -> $ret_type:ty
            }
        ),* // Comma separator between blocks {}
        $(,)? // Optional trailing comma after the last block
    ) => {
        $( // Expansion repetition matches the block repetition
            #[wasm_bindgen::prelude::wasm_bindgen] // Using #[wasm_bindgen] directly often works
            pub async fn $func_name( $( $arg_name : $arg_type ),* ) -> $ret_type {
                // Call the namespaced function, specify generic, pass args, and await
        ($namespace :: $func_name :: <$GenericType> ( $( $arg_name ),*) ).await
            }
        )*
    };
}

// --- Corrected Macro Invocation ---
specialize_functions!(
    api_request::note, // Namespace path
    DefaultBackend;   // Generic Type

    // Function specifications within blocks {}
    {
        get_note,
        ( // Args for get_note
            note: NoteID,
            store: &str,
            cookie: &str, // Trailing comma inside args is OK now
        ), // End args for get_note
        -> Result<(Metadata, Note), Error> // Return type for get_note
    }, // Separator comma

    {
        add_new_note,
        ( // Args for add_new_note
            note: &Note,
            session: &Session,
        ), // End args for add_new_note
        -> Result<Metadata, Error> // Return type for add_new_note
    }, // Separator comma

    {
        update_note,
        ( // Args for update_note
            note: &NoteUpdate,
            session: &Session,
        ), // End args for update_note
        // NO extra comma here anymore
        -> Result<(Metadata, Option<Note>), Error> // Return type for update_note
    }, // Separator comma

    {
        delete_note,
        ( // Args for delete_note
            note: &NoteID,
            session: &Session,
        ), // End args for delete_note
        -> Result<Metadata, Error> // Return type for delete_note
    }, // Separator comma

    {
        search_notes,
        ( // Args for search_notes
            search: &NoteSearch,
            session: &Session,
        ), // End args for search_notes
        -> Result<(Metadata, Vec<NoteID>), Error> // Return type for search_notes
    }, // Separator comma

    {
        get_all_notes,
        ( // Args for get_all_notes
            session: &Session,
        ), // End args for get_all_notes
        -> Result<(Metadata, Vec<NoteID>), Error> // Return type for get_all_notes
    } // No comma needed after the last block

    // Optional trailing comma could go here: ,
);
