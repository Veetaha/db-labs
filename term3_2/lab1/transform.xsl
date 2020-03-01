<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns="http://www.w3.org/1999/xhtml">
    <xsl:output
        method="xml"
        doctype-system="http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd"
        doctype-public="-//W3C//DTD XHTML 1.1//EN"
        indent="yes"
    />
    <xsl:template match="/">
        <html xml:lang="en">
            <head>
                <title>LABA</title>
            </head>
            <body>
                <h1>PRODUCTS:</h1>
                <xsl:apply-templates select="/shop"/>
            </body>
        </html>
    </xsl:template>
    <xsl:template match="/shop">
        <table border="1">
            <thead>
                <tr>
                    <td>UAH</td>
                    <td>TEXT</td>
                    <td>AVAIL</td>
                </tr>
            </thead>
            <tbody>
                <xsl:apply-templates select="/shop/product"/>
            </tbody>
        </table>
    </xsl:template>
    <xsl:template match="/shop/product">
        <tr>
            <td>
                <xsl:apply-templates select="image"/>
                <xsl:apply-templates select="description"/>
            </td>
            <td><xsl:apply-templates select="price"/></td>
            <td><xsl:apply-templates select="@availability"/></td>
        </tr>
    </xsl:template>
    <xsl:template match="image">
        <img>
            <xsl:attribute name="src">
                <xsl:value-of select="text()"/>
            </xsl:attribute>
        </img>
    </xsl:template>
    <xsl:template match="price">
        <xsl:attribute name="style">
            font-weight: bold
        </xsl:attribute>
        <xsl:value-of select="text()"/>
    </xsl:template>
    <xsl:template match="description">
        <xsl:attribute name="style">
            font-size: 20px
        </xsl:attribute>
        <xsl:value-of select="text()"/>
    </xsl:template>
    <xsl:template match="@availability">
        <xsl:value-of select="."></xsl:value-of>
    </xsl:template>
</xsl:stylesheet>
