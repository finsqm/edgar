SELECT
    production.product.name,
    production.product_subcategory.name,
    production.productcategory.name FROM production.product
JOIN production.product_subcategory
    ON
        production.product.productsubcategoryid
        = production.product_subcategory.productsubcategoryid
JOIN production.productcategory
    ON
        production.product_subcategory.productcategoryid
        = production.productcategory.productcategoryid
WHERE production.product.class = 'L'
