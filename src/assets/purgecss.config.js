module.exports = {
    safelist: [
        /* General html tags that may arise from parsing markdown */
        /ol/,
        /ul/,
        /li/,
        /h1/,
        /h2/,
        /h3/,
        /h4/,
        /h5/,
        /h6/,
        /p/,
        /img/,
        /blockquote/,
        /code/,
        /pre/,
        /a/,
        /table/,
        /th/,
        /thead/,
        /tr/,
        /tbody/,
        /td/,
        /hr/,
        /span/,
        /body/,
        /nav/,
        /html/,
        /dt/,
        /dd/,
        /input/,
        // classes need for fancy checkboxes
        /(^|.)form-check.*$/,
    ],
    variables: true,
};
