// on document load.
$(document).ready(function () {
    // call feather replace to load all feather icons once the document is loaded.
    feather.replace();

    // for all spinner buttons
    $(".btn-spinner").each(function () {
        let submit_button = $(this);
        // get the parent form.
        // set submit to start the loading animation.
        submit_button.parents("form:first").submit(function () {
            // disable the button
            submit_button.prop("disabled", true);
            // set to loading animation.
            submit_button.html(
                `<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>`
            );
        });
    });

});

