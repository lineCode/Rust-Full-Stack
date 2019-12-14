pub async fn reference<SD>(
    data: web::Data<ServerData>,
    path: web::Path<(String,)>,
    req: HttpRequest,
) -> Result<HttpResponse, HtmlError>
where
    SD: SwordDrillable,
{
    let db = data.db.to_owned();
    let raw_reference = path.0.replace("/", ".");
    match raw_reference.parse::<Reference>() {
        Ok(reference) => {
            let data_reference = reference.to_owned();
            let result =
                web::block(move || SD::verses(&reference, &VerseFormat::HTML, &db.get().unwrap()))
                    .await??;
            let verses_data = VersesData::new(result, data_reference, &req);

            if verses_data.verses.is_empty() {
                return Err(Error::InvalidReference {
                    reference: raw_reference,
                }
                .into());
            }

            let body = TemplateData::new(
                &verses_data,
                Meta::for_reference(
                    &verses_data.reference,
                    &verses_data.verses,
                    &verses_data.links,
                ),
            )
            .to_html("chapter", &data.template)?;
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        Err(_) => Err(HtmlError(Error::InvalidReference {
            reference: raw_reference,
        })),
    }
}

