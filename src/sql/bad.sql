select a.Name,b.Name,c.Name from production.product as a 
join production.ProductSubcategory as b
on a.productsubcategoryid = b.productsubcategoryid
join production.productcategory as c
on b.productcategoryid = c.productcategoryid
where a.class = 'L'