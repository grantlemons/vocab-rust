#[tokio::test]
async fn test_delantal() -> Result<(), String> {
    match crate::get_defs("delantal".to_string(), None, None).await {
        Ok(res) => {
            assert_eq!(res.definitions.len(), 2);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[tokio::test]
async fn test_entregar() -> Result<(), String> {
    match crate::get_defs("entregar".to_string(), None, None).await {
        Ok(res) => {
            assert_eq!(res.definitions.len(), 11);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[tokio::test]
async fn test_nuevo() -> Result<(), String> {
    match crate::get_defs("nuevo".to_string(), None, None).await {
        Ok(res) => {
            assert_eq!(res.definitions.len(), 4);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[tokio::test]
async fn test_palabra() -> Result<(), String> {
    match crate::get_defs("palabra".to_string(), None, None).await {
        Ok(res) => {
            assert_eq!(res.definitions.len(), 2);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[tokio::test]
async fn test_burgues() -> Result<(), String> {
    match crate::get_defs("burgués".to_string(), None, None).await {
        Ok(res) => {
            assert_eq!(res.definitions.len(), 2);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[tokio::test]
async fn test_random_words() -> Result<(), String> {
    /* cspell: disable */
    let words: &[&str] = &[
        "ladrillo",
        "destilación",
        "prolongado",
        "alto",
        "verte",
        "hace",
        "ya",
        "puedo",
    ];
    /* cspell: enable */
    for word in words {
        crate::get_defs(word.to_string(), None, None).await?;
    }
    Ok(())
}

#[tokio::test]
async fn test_invalid_word() -> Result<(), String> {
    match crate::get_defs("sjfadohjfahndkllhjra".to_string(), None, None).await {
        Ok(_) => Err("Expected error".to_string()),
        Err(_) => Ok(()),
    }
}

#[tokio::test]
async fn english_to_spanish_brick() -> Result<(), String> {
    match crate::get_defs("brick".to_string(), None, None).await {
        Ok(res) => {
            assert_eq!(res.definitions.len(), 3);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
